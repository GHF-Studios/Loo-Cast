using UnityEngine;

namespace LooCast.Noise
{
    public static class PerlinNoise
    {
        public static float[,] GenerateNoiseMap(int mapWidth, int mapHeight, int seed, float scale, int octaves, float persistence, float lacunarity, float amplitude, Vector2 offset)
        {
            float[,] noiseMap = new float[mapWidth, mapHeight];

            System.Random prng = new System.Random(seed);
            Vector2[] octaveOffsets = new Vector2[octaves];
            for (int i = 0; i < octaves; i++)
            {
                float offsetX = prng.Next(-100000, 100000) + offset.x;
                float offsetY = prng.Next(-100000, 100000) + offset.y;
                octaveOffsets[i] = new Vector2(offsetX, offsetY);
            }

            if (scale <= 0)
            {
                scale = 0.0001f;
            }

            float halfWidth = mapWidth / 2.0f;
            float halfHeight = mapHeight / 2.0f;

            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    float sampleAmplitude = 1.0f;
                    float sampleFrequency = 1.0f;
                    float noiseHeight = 0.0f;

                    for (int i = 0; i < octaves; i++)
                    {
                        float sampleX = ((x - halfWidth + octaveOffsets[i].x) / scale) * sampleFrequency;
                        float sampleY = ((y - halfHeight + octaveOffsets[i].y) / scale) * sampleFrequency;
    
                        noiseHeight += (Mathf.PerlinNoise(sampleX, sampleY) * 2 - 1) * sampleAmplitude;

                        sampleAmplitude *= persistence;
                        sampleFrequency *= lacunarity;
                    }

                    noiseMap[x, y] = noiseHeight;
                }
            }

            float maxNoiseAmplitude = 0.0f;
            float noiseAmplitude = 1.0f;
            for (int i = 0; i < octaves; i++)
            {
                maxNoiseAmplitude += noiseAmplitude;
                noiseAmplitude *= persistence;
            }

            maxNoiseAmplitude *= 1 / amplitude;

            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    noiseMap[x, y] = Mathf.InverseLerp(-maxNoiseAmplitude, maxNoiseAmplitude, noiseMap[x, y]);
                }
            }

            return noiseMap;
        }
    } 
}
