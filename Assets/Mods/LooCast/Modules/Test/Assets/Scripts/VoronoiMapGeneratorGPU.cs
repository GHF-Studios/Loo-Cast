using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Util;
    using LooCast.Noise;

    public class VoronoiMapGeneratorGPU : MonoBehaviour
    {
        public int MapWidth;
        public int MapHeight;
        public Vector2Int SampleCellAmount;
        [Range(0.0f, 2.0f)]
        public float CellSpread;
        public float Power;
        public float Amplitude;
        [Range(0, 2109876543)]
        public int Seed;
        public bool AutoUpdate;
        public ComputeShader ComputeShader;

        public void GenerateMap()
        {
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
                colorValue = Mathf.Pow(colorValue, Power);
                colorValue *= Amplitude;
                colorMap[i] = new Color(colorValue, colorValue, colorValue, 1.0f);
            }
            #endregion

            #region Pixel Data Display
            MapDisplay display = GetComponent<MapDisplay>();
            display.DrawTexture(TextureUtil.TextureFromColorMap(colorMap, MapWidth, MapHeight));
            #endregion
        }

        private float GetMaxDistance(float[] distances)
        {
            float maxDistance = float.MinValue;
            for (int i = 0; i < distances.Length; i++)
            {
                if (distances[i] > maxDistance)
                {
                    maxDistance = distances[i];
                }
            }
            return maxDistance;
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
            
            if (SampleCellAmount.x < 1)
            {
                SampleCellAmount.x = 1;
            }
            if (SampleCellAmount.y < 1)
            {
                SampleCellAmount.y = 1;
            }
        }

        #region Structs
        private struct VoronoiCell
        {
            int index;
            int positionX;
            int positionY;

            public VoronoiCell(int index, int positionX, int positionY)
            {
                this.index = index;
                this.positionX = positionX;
                this.positionY = positionY;
            }

            public static int ByteSize
            {
                get
                {
                    return sizeof(int) * 3;
                }
            }
        }

        private struct VoronoiPixel
        {
            public float ClosestVoronoiCellIndex
            {
                get
                {
                    return closestVoronoiCellIndex;
                }
            }
            public float DistanceToVoronoiCell
            {
                get
                {
                    return distanceToVoronoiCell;
                }
            }

            private int positionX;
            private int positionY;
            private int closestVoronoiCellIndex;
            private float distanceToVoronoiCell;

            public VoronoiPixel(int positionX, int positionY)
            {
                this.positionX = positionX;
                this.positionY = positionY;
                closestVoronoiCellIndex = 0;
                distanceToVoronoiCell = 0.0f;
            }

            public static int ByteSize
            {
                get
                {
                    return (sizeof(int) * 3) + (sizeof(float));
                }
            }
        }
        #endregion
    }
}
