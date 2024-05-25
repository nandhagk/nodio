#include "RtAudio.h"
#include <cstdint>
#include <iostream>
#include <cstdlib>
#include <cstring>
#include <functional>

using Amplitude_t = double;
#define FORMAT RTAUDIO_FLOAT64
#define BUFFER_FRAMES 512
#define CHANNELS 2
unsigned int bufferBytes = BUFFER_FRAMES * CHANNELS * sizeof(Amplitude_t);
#define BUFFER_SIZE (bufferBytes / sizeof(Amplitude_t))

class Transformer{
public:
	virtual void transform(Amplitude_t* output, Amplitude_t* input) = 0;
};

int inout(void *outputBuffer, void *inputBuffer, unsigned int /*nBufferFrames*/,
		  double streamTime, RtAudioStreamStatus status, void *data)
{
	// Since the number of input and output channels is equal, we can do
	// a simple buffer copy operation here.
	if (status)
		std::cout << "Stream over/underflow detected." << std::endl;

	std::vector<Transformer*> transformers = *(static_cast<std::vector<Transformer*> *>(data));
	for (auto transformer : transformers)
	{
		transformer->transform(static_cast<Amplitude_t *>(outputBuffer), static_cast<Amplitude_t *>(inputBuffer));
	}
	return 0;
}

class Amplifier: virtual public Transformer{
public:
	int amplification_constant;
	Amplifier(int amplification_constant): amplification_constant(amplification_constant){
		
	}
	void transform(Amplitude_t* output, Amplitude_t* input){
		for (int i = 0; i < BUFFER_SIZE; ++i){
			output[i] = input[i] * amplification_constant;
		}
	}
};


void delay(Amplitude_t* output, Amplitude_t* input){

}

int main(int argc, char *argv[])
{
	RtAudio adac;
	std::vector<unsigned int> deviceIds = adac.getDeviceIds();
	if (deviceIds.size() < 1)
	{
		std::cerr << "\nERROR! No audio devices found!\n";
		exit(1);
	}

	unsigned int channels = CHANNELS;
	unsigned int sample_rate = 48000;

	adac.showWarnings(true);

	unsigned int bufferFrames = BUFFER_FRAMES;
	RtAudio::StreamParameters iParams, oParams;
	iParams.nChannels = channels;
	iParams.firstChannel = 0;
	oParams.nChannels = channels;
	oParams.firstChannel = 0;

	iParams.deviceId = adac.getDefaultInputDevice();
	oParams.deviceId = adac.getDefaultOutputDevice();

	RtAudio::StreamOptions options;

	std::vector<Transformer*> transformers;
	transformers.push_back(new Amplifier(1));

	if (adac.openStream(&oParams, &iParams, FORMAT, sample_rate, &bufferFrames, &inout, (void *)&transformers, &options))
	{
		goto cleanup;
	}

	if (adac.startStream())
		goto cleanup;

	char input;
	std::cout << "\nRunning ... press <enter> to quit (buffer frames = " << bufferFrames << ").\n";
	std::cin.get(input);

	// Stop the stream.
	if (adac.isStreamRunning())
		adac.stopStream();

cleanup:
	if (adac.isStreamOpen())
		adac.closeStream();

	return 0;
}