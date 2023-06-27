use aspeak::{
    get_rest_endpoint_by_region, AudioFormat, AuthOptionsBuilder, SynthesizerConfig,
    TextOptionsBuilder,
};

use std::error::Error;

use tokio::{fs::File, io::AsyncWriteExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let auth = AuthOptionsBuilder::new(
        get_rest_endpoint_by_region("eastus"), // Use eastus endpoint, RESTful API
    )
    .key("YOUR_AZURE_SUBSCRIPTION_KEY")
    .build();
    let config = SynthesizerConfig::new(auth, AudioFormat::Riff16Khz16BitMonoPcm);
    let rest_syn = config.rest_synthesizer()?;
    let ssml = r#"<speak version="1.0" xmlns="http://www.w3.org/2001/10/synthesis" xml:lang="en-US"><voice name="en-US-JennyNeural">Hello, world!</voice></speak>"#;
    let audio_data = rest_syn.synthesize_ssml(ssml).await?;
    let mut file = File::create("ssml-output.wav").await?;
    file.write_all(&audio_data).await?;
    let text = "Hello, world!";
    let options = TextOptionsBuilder::new()
        .voice("en-US-JennyNeural")
        .rate("fast")
        .pitch("high")
        .build();
    let audio_data = rest_syn.synthesize_text(text, &options).await?;
    let mut file = File::create("text-output.wav").await?;
    file.write_all(&audio_data).await?;
    Ok(())
}
