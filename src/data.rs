use async_trait::async_trait;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::{Float, FreeFallObjectSnapshot};

const MAX_DATAPOINTS: usize = 2000;

#[async_trait]
pub trait Data<const D: usize, const N: usize, AppliedType, UserType> {
    fn new_datapoint(
        time: Float,
        object: &FreeFallObjectSnapshot<D>,
        applied: &AppliedType,
        user: &UserType,
    ) -> [Float; N];
    fn column_names() -> [&'static str; N];
    fn should_end(
        time: Float,
        object: &FreeFallObjectSnapshot<D>,
        applied: &AppliedType,
        current_data: &[[Float; N]],
        user: &UserType,
    ) -> bool;
    async fn write_data<W: AsyncWrite + Unpin + Send>(data: &[[Float; N]], output: &mut W) {
        let mut output_writer = BufWriter::new(output);

        let first_row = {
            let mut b = Self::column_names().join(", ");
            b.push('\n');
            b
        };

        output_writer.write_all(first_row.as_bytes()).await.unwrap();

        if data.len() > MAX_DATAPOINTS {
            let mut index = 0.0;

            let step_size = data.len() as f32 / MAX_DATAPOINTS as f32;

            while let Some(datapoint) = data.get(index as usize) {
                write_datapoint(&mut output_writer, *datapoint).await;
                index += step_size;
            }
        } else {
            for datapoint in data {
                write_datapoint(&mut output_writer, *datapoint).await;
            }
        }

        output_writer.flush().await.unwrap()
    }
}

async fn write_datapoint<W: AsyncWrite + Unpin, const N: usize>(
    output: &mut W,
    datapoint: [Float; N],
) {
    let mut buf = datapoint.map(|x| ToString::to_string(&x)).join(", ");
    buf.push('\n');

    output.write_all(buf.as_bytes()).await.unwrap();
}
