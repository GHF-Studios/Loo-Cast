using UnityEngine;

namespace LooCast.Noise
{
    using LooCast.Random;

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
        public int regionAmount;

        public int seed;

        public bool autoUpdate;

        public void GenerateMap()
        {
            MapDisplay display = GetComponent<MapDisplay>();
            if (drawMode == DrawMode.Distance)
            {
                display.DrawTexture(TextureGenerator.TextureFromColorMap(GetDistanceColorMap(), mapWidth, mapHeight));
            }

            else if (drawMode == DrawMode.Cell)
            {
                display.DrawTexture(TextureGenerator.TextureFromColorMap(GetCellColorMap(), mapWidth, mapHeight));
            }
        }

        private Vector2Int[] GetCentroids()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = new Vector2Int[regionAmount];
            for (int i = 0; i < regionAmount; i++)
            {
                centroids[i] = new Vector2Int(prng.Range(0, mapWidth), prng.Range(0, mapHeight));
            }
            return centroids;
        }

        private Color[] GetCellColorMap()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = GetCentroids();
            Color[] centroidColors = new Color[regionAmount];
            for (int i = 0; i < regionAmount; i++)
            {
                centroidColors[i] = prng.Color();
            }

            Color[] colorMap = new Color[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    colorMap[y * mapWidth + x] = centroidColors[GetClosestCentroidIndex(new Vector2Int(x, y), centroids)];
                }
            }

            return colorMap;
        }

        private Color[] GetDistanceColorMap()
        {
            SeededRandom prng = new SeededRandom(seed);
            Vector2Int[] centroids = GetCentroids();

            Color[] colorMap = new Color[mapWidth * mapHeight];
            float[] distances = new float[mapWidth * mapHeight];
            for (int y = 0; y < mapHeight; y++)
            {
                for (int x = 0; x < mapWidth; x++)
                {
                    distances[y * mapWidth + x] = Vector2.Distance(new Vector2Int(x, y), centroids[GetClosestCentroidIndex(new Vector2Int(x, y), centroids)]);
                }
            }
            float maxDistance = GetMaxDistance(distances);
            for (int i = 0; i < distances.Length; i++)
            {
                float colorValue = distances[i] / maxDistance;
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
        }
    } 
}
