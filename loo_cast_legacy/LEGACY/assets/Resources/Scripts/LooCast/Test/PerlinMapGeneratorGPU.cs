using CSSystem = System;
using UnityEngine;

namespace LooCast.Test
{
    using Noise;
    using Util;

    public class PerlinMapGeneratorGPU : MonoBehaviour
    {
        public ComputeShader ComputeShader;
        public DrawMode CurrentDrawMode;

        public int MapWidth;
        public int MapHeight;
        public float NoiseScale;
        public float NoiseAmplitude;
        public int Octaves;
        [Range(0.0f, 1.0f)]
        public float Persistence;
        public float Lacunarity;
        [Range(0, 2109876543)]
        public int Seed;
        public Vector2 Offset;
        public bool AutoUpdate;
        public TerrainType[] Regions;

        private const int maxOctaves = 5;

        public void GenerateMap()
        {
            #region Perlin Pixel Compute Buffer Creation
            PerlinPixel[] perlinPixelsData = new PerlinPixel[MapWidth * MapHeight];
            for (int y = 0; y < MapHeight; y++)
            {
                for (int x = 0; x < MapWidth; x++)
                {
                    perlinPixelsData[y * MapWidth + x] = new PerlinPixel(x, y);
                }
            }

            ComputeBuffer perlinPixelsBuffer = new ComputeBuffer(perlinPixelsData.Length, PerlinPixel.ByteSize);
            perlinPixelsBuffer.SetData(perlinPixelsData);
            #endregion

            #region Compute Shader Creation and Execution
            ComputeShader.SetBuffer(1, "perlinPixels", perlinPixelsBuffer);
            ComputeShader.SetInts("textureDimensions", MapWidth, MapHeight);
            ComputeShader.SetInt("seed", Seed);
            ComputeShader.SetFloat("scale", NoiseScale);
            ComputeShader.SetInt("octaves", Octaves);
            ComputeShader.SetFloat("persistence", Persistence);
            ComputeShader.SetFloat("lacunarity", Lacunarity);
            ComputeShader.SetFloat("amplitude", NoiseAmplitude);
            ComputeShader.Dispatch(0, 1, 1, 1);
            ComputeShader.Dispatch(1, MapWidth / 32, MapHeight / 32, 1);
            perlinPixelsBuffer.GetData(perlinPixelsData);
            perlinPixelsBuffer.Dispose();
            #endregion

            #region Pixel Data Evaluation & Display
            MapDisplay display = GetComponent<MapDisplay>();
            if (CurrentDrawMode == DrawMode.NoiseMap)
            {
                Color[] noiseMap = new Color[perlinPixelsData.Length];
                for (int y = 0; y < MapHeight; y++)
                {
                    for (int x = 0; x < MapWidth; x++)
                    {
                        float perlinValue = perlinPixelsData[y * MapWidth + x].PerlinValue;
                        noiseMap[y * MapWidth + x] = new Color(perlinValue, perlinValue, perlinValue, 1.0f);
                    }
                }

                display.DrawTexture(TextureUtil.TextureFromColorMap(noiseMap, MapWidth, MapHeight));
            }
            else if (CurrentDrawMode == DrawMode.ColorMap)
            {
                Color[] colorMap = new Color[perlinPixelsData.Length];
                for (int y = 0; y < MapHeight; y++)
                {
                    for (int x = 0; x < MapWidth; x++)
                    {
                        float currentHeight = perlinPixelsData[y * MapWidth + x].PerlinValue;
                        for (int i = 0; i < Regions.Length; i++)
                        {
                            if (currentHeight <= Regions[i].height)
                            {
                                colorMap[y * MapWidth + x] = Regions[i].color;
                                break;
                            }
                        }
                    }
                }

                display.DrawTexture(TextureUtil.TextureFromColorMap(colorMap, MapWidth, MapHeight));
            }
            #endregion
        }

        private void OnValidate()
        {
            if ((MapWidth % 32) != 0)
            {
                MapWidth -= (MapWidth % 32);
            }
            if (MapWidth < 32)
            {
                MapWidth = 32;
            }

            if ((MapHeight % 32) != 0)
            {
                MapHeight -= (MapHeight % 32);
            }
            if (MapHeight < 32)
            {
                MapHeight = 32;
            }

            if (Seed < 0)
            {
                Seed = Mathf.Abs(Seed);
            }

            if (MapWidth < 1)
            {
                MapWidth = 1;
            }
            if (MapHeight < 1)
            {
                MapHeight = 1;
            }
            if (Lacunarity < 1.0f)
            {
                Lacunarity = 1.0f;
            }
            if (Octaves < 0)
            {
                Octaves = 0;
            }
            if (Octaves > maxOctaves)
            {
                Octaves = maxOctaves;
            }
        }

        #region Enums
        public enum DrawMode
        {
            NoiseMap,
            ColorMap
        }
        #endregion

        #region Structs
        [CSSystem.Serializable]
        public struct TerrainType
        {
            public string name;
            public float height;
            public Color color;
        }

        private struct PerlinPixel
        {
            public float PerlinValue => perlinValue;

            private int positionX;
            private int positionY;
            private float perlinValue;

            public PerlinPixel(int positionX, int positionY)
            {
                this.positionX = positionX;
                this.positionY = positionY;
                perlinValue = 0.0f;
            }

            public static int ByteSize
            {
                get
                {
                    return (sizeof(int) * 2) + (sizeof(float));
                }
            }
        }
        #endregion
    }
}
