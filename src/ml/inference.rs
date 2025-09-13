use tch::{CModule, Device, Tensor};

pub fn score_clips(frames: &Vec<Vec<u8>>, audio: &Vec<f32>) -> anyhow::Result<Vec<(usize, f32)>> {
    // TODO: Load pretrained model with tch-rs and score frames
    Ok(vec![])
}

fn preprocess_frame(frame: &Vec<u8>) -> anyhow::Result<Tensor> {
    // TODO: Convert raw frame bytes -> Tensor
    Ok(Tensor::zeros(
        &[3, 224, 224],
        (tch::Kind::Uint8, Device::Cpu),
    ))
}
