use async_trait::async_trait;
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};

use crate::{BodySnapshot, Float};

const MAX_DATAPOINTS: usize = 2000;

#[async_trait]
pub trait DataLogger<
    const D: usize,
    const N: usize,
    StepType,
    UserType,
    W: AsyncWrite + Send + Sync + Unpin,
>
{
    /// The `object` parameter is the object before the step
    fn new_datapoint(
        &mut self,
        time: Float,
        object: &BodySnapshot<D>,
        step: &StepType,
        user: &UserType,
    ) -> [Float; N];
    fn column_names() -> [&'static str; N];
    fn should_end(
        &mut self,
        time: Float,
        object: &BodySnapshot<D>,
        step: &StepType,
        current_data: &[[Float; N]],
        user: &UserType,
    ) -> bool;
    fn get_output(&mut self) -> &mut W;
    async fn write_data(data: &[[Float; N]], output: &mut W) {
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

            if index as usize != data.len() - 1 {
                write_datapoint(&mut output_writer, *data.last().unwrap()).await;
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
