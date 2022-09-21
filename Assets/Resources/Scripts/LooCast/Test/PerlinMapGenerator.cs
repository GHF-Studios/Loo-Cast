using UnityEngine;

namespace LooCast.Test
{
    using Noise;
    using Util;

    public class PerlinMapGenerator : MonoBehaviour
    {
        public enum DrawMode
        {
            NoiseMap,
            ColorMap
        }
        public DrawMode drawMode;

        public int mapWidth;
        public int mapHeight;
        public float noiseScale;
        public float noiseAmplitude;

        public int octaves;
        [Range(0.0f, 1.0f)]
        public float persistence;
        public float lacunarity;

        [Range(0, 2109876543)]
        public int seed;
        public Vector2 offset;

        public bool autoUpdate;

        public TerrainType[] regions;

        public void GenerateMap()
        {
            float[,] noiseMap = PerlinNoise.GenerateNoiseMap(mapWidth, mapHeight, seed, noiseScale, octaves, persistence, lacunarity, noiseAmplitude, offset);

            Color[] colorMap = new Color[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    float currentHeight = noiseMap[x, y];
                    for (int i = 0; i < regions.Length; i++)
                    {
                        if (currentHeight <= regions[i].height)
                        {
                            colorMap[y * mapWidth + x] = regions[i].color;
                            break;
                        }
                    }
                }
            }

            MapDisplay display = GetComponent<MapDisplay>();
            if (drawMode == DrawMode.NoiseMap)
            {
                display.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap));
            }

            else if (drawMode == DrawMode.ColorMap)
            {
                display.DrawTexture(TextureUtil.TextureFromColorMap(colorMap, mapWidth, mapHeight));
            }
        }

        private void OnValidate()
        {
            if (mapWidth < 1)
            {
                mapWidth = 1;
            }
            if (mapHeight < 1)
            {
                mapHeight = 1;
            }
            if (lacunarity < 1)
            {
                lacunarity = 1;
            }
            if (octaves < 0)
            {
                octaves = 0;
            }
        }
    } 

    [System.Serializable]
    public struct TerrainType
    {
        public string name;
        public float height;
        public Color color;
    }
}
