using UnityEngine;

namespace LooCast.Test
{
    using LooCast.Random;
    using LooCast.Util;
    using LooCast.Noise;

    public class VoronoiMapGenerator : MonoBehaviour
    {
        public enum DrawMode
        {
            Distance,
            Cell
        }
        public DrawMode drawMode;


        public int mapWidth;
        public int mapHeight;
        public Vector2Int sampleCellAmount;
        public Vector2Int offset;

        [Range(0.0f, 2.0f)]
        public float cellSpread;
        public float power;
        public float amplitude;

        [Range(0, 2109876543)]
        public int seed;

        public bool autoUpdate;

        public void GenerateMap()
        {
            MapDisplay display = GetComponent<MapDisplay>();
            if (drawMode == DrawMode.Distance)
            {
                display.DrawTexture(TextureUtil.TextureFromColorMap(GetDistanceColorMap(), mapWidth, mapHeight));
            }

            else if (drawMode == DrawMode.Cell)
            {
                display.DrawTexture(TextureUtil.TextureFromColorMap(GetCellColorMap(), mapWidth, mapHeight));
            }
        }

        private Color[] GetCellColorMap()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = VoronoiNoise.GetCentroids(seed, mapWidth, mapHeight, sampleCellAmount, cellSpread);

            Color[] centroidColors = new Color[sampleCellAmount.x * sampleCellAmount.y];
            for (int i = 0; i < centroidColors.Length; i++)
            {
                centroidColors[i] = prng.Color();
            }

            Color[] colorMap = new Color[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    int currentPixelIndex = y * mapWidth + x;
                    Vector2Int currentPixelPosition = new Vector2Int(x + offset.x, y + offset.y);
                    int closestCentroidIndex = GetClosestCentroidIndex(currentPixelPosition, centroids);
                    Vector2Int closestCentroid = centroids[closestCentroidIndex];
                    if (closestCentroid == currentPixelPosition)
                    {
                        colorMap[currentPixelIndex] = Color.black;
                    }
                    else
                    {
                        colorMap[currentPixelIndex] = centroidColors[closestCentroidIndex];     
                    }
                }
            }

            return colorMap;
        }

        private Color[] GetDistanceColorMap()
        {
            Vector2Int[] centroids = VoronoiNoise.GetCentroids(seed, mapWidth, mapHeight, sampleCellAmount, cellSpread);
            float[] distances = new float[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    distances[y * mapWidth + x] = Vector2.Distance(new Vector2Int(x, y), centroids[GetClosestCentroidIndex(new Vector2Int(x, y), centroids)]);
                }
            }

            Color[] colorMap = new Color[mapWidth * mapHeight];
            float maxDistance = GetMaxDistance(distances);
            for (int i = 0; i < distances.Length; i++)
            {
                float colorValue = distances[i] / maxDistance;
                colorValue = Mathf.Pow(colorValue, (1 - colorValue) * power);
                colorValue *= amplitude;
                colorMap[i] = new Color(colorValue, colorValue, colorValue, 1.0f);
            }

            return colorMap;
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

        private int GetClosestCentroidIndex(Vector2Int pixelPos, Vector2Int[] centroids)
        {
            float minDistance = float.MaxValue;
            int centroidIndex = 0;
            for (int i = 0; i < centroids.Length; i++)
            {
                float distance = Vector2.Distance(pixelPos, centroids[i]);
                if (distance < minDistance)
                {
                    minDistance = distance;
                    centroidIndex = i;
                }
            }
            return centroidIndex;
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
            if (sampleCellAmount.x < 1)
            {
                sampleCellAmount.x = 1;
            }
            if (sampleCellAmount.y < 1)
            {
                sampleCellAmount.y = 1;
            }
        }
    } 
}
