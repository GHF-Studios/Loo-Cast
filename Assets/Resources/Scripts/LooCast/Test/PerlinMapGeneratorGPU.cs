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
        public TerrainType[] regions;

        public ComputeShader ComputeShader;

        public void GenerateMap()
        {
            #region OLD DUSTY ASS CPU ALGORITHM
            float[,] noiseMap = PerlinNoise.GenerateNoiseMap(MapWidth, MapHeight, Seed, NoiseScale, Octaves, Persistence, Lacunarity, NoiseAmplitude, Offset).Array2D;

            Color[] colorMap = new Color[MapWidth * MapHeight];
            for (int y = 0; y < MapHeight; y++)
            {
                for (int x = 0; x < MapWidth; x++)
                {
                    float currentHeight = noiseMap[x, y];
                    for (int i = 0; i < regions.Length; i++)
                    {
                        if (currentHeight <= regions[i].height)
                        {
                            colorMap[y * MapWidth + x] = regions[i].color;
                            break;
                        }
                    }
                }
            }

            MapDisplay display = GetComponent<MapDisplay>();
            if (CurrentDrawMode == DrawMode.NoiseMap)
            {
                display.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap));
            }

            else if (CurrentDrawMode == DrawMode.ColorMap)
            {
                display.DrawTexture(TextureUtil.TextureFromColorMap(colorMap, MapWidth, MapHeight));
            }
            #endregion

            #region Voronoi Cell Compute Buffer Creation
            Vector2Int[] voronoiCells = VoronoiNoise.GetCentroids(Seed, MapWidth, MapHeight, SampleCellAmount, CellSpread);
            VoronoiCell[] voronoiCellsData = new VoronoiCell[voronoiCells.Length];
            for (int i = 0; i < voronoiCellsData.Length; i++)
            {
                voronoiCellsData[i] = new VoronoiCell(i, voronoiCells[i].x, voronoiCells[i].y);
            }

            ComputeBuffer voronoiCellsBuffer = new ComputeBuffer(voronoiCellsData.Length, VoronoiCell.ByteSize);
            voronoiCellsBuffer.SetData(voronoiCellsData);
            #endregion

            #region Voronoi Pixel Compute Buffer Creation
            VoronoiPixel[] voronoiPixelsData = new VoronoiPixel[MapWidth * MapHeight];
            for (int y = 0; y < MapHeight; y++)
            {
                for (int x = 0; x < MapWidth; x++)
                {
                    voronoiPixelsData[y * MapWidth + x] = new VoronoiPixel(x, y);
                }
            }

            ComputeBuffer voronoiPixelsBuffer = new ComputeBuffer(voronoiPixelsData.Length, VoronoiPixel.ByteSize);
            voronoiPixelsBuffer.SetData(voronoiPixelsData);
            #endregion

            #region Compute Shader Creation and Execution
            ComputeShader.SetBuffer(0, "voronoiCells", voronoiCellsBuffer);
            ComputeShader.SetBuffer(0, "voronoiPixels", voronoiPixelsBuffer);
            ComputeShader.SetInts("textureDimensions", MapWidth, MapHeight);
            ComputeShader.SetInts("voronoiCellArrayDimensions", SampleCellAmount.x, SampleCellAmount.y);
            ComputeShader.Dispatch(0, MapWidth / 32, MapHeight / 32, 1);
            voronoiPixelsBuffer.GetData(voronoiPixelsData);
            voronoiCellsBuffer.Dispose();
            voronoiPixelsBuffer.Dispose();
            #endregion

            #region Computed Pixel Data Evaluation
            float[] distances = new float[voronoiPixelsData.Length];
            for (int i = 0; i < distances.Length; i++)
            {
                distances[i] = voronoiPixelsData[i].DistanceToVoronoiCell;
            }

            Color[] colorMap = new Color[voronoiPixelsData.Length];
            float maxDistance = GetMaxDistance(distances);
            for (int i = 0; i < distances.Length; i++)
            {
                float colorValue = distances[i] / maxDistance;
                colorValue = Mathf.Pow(colorValue, (1 - colorValue) * Power);
                colorValue *= Amplitude;
                colorMap[i] = new Color(colorValue, colorValue, colorValue, 1.0f);
            }
            #endregion

            #region Pixel Data Display
            MapDisplay display = GetComponent<MapDisplay>();
            display.DrawTexture(TextureUtil.TextureFromColorMap(colorMap, MapWidth, MapHeight));
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
            if (Lacunarity < 1)
            {
                Lacunarity = 1;
            }
            if (Octaves < 0)
            {
                Octaves = 0;
            }
        }
    } 

    #region Structs
    [System.Serializable]
    public struct TerrainType
    {
        public string name;
        public float height;
        public Color color;
    }

    private struct PerlinPixel
    {
        private int positionX;
        private int positionY;
        private float perlinValue;

        public VoronoiPixel(int positionX, int positionY)
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
