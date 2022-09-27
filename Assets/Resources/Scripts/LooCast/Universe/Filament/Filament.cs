using UnityEngine;
using System;

namespace LooCast.Universe.Filament
{
    using LooCast.Test;
    using LooCast.Util;
    using LooCast.Random;

    [Serializable]
    public class Filament
    {
        #region Structs
        [Serializable]
        public struct GenerationSettings
        {
            public GameObject prefab;
            // How many Sectors fit into a Filament (Per Axis)
            public int size;
        }
        #endregion

        public Vector2Int WorldPosition => worldPosition;
        public Vector2Int FilamentPosition => filamentPosition;
        public Texture2D Map
        {
            get
            {
                return map;
            }

            set
            {

            }
        }

        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject filamentObject;
        private Texture2D map;

        public Filament(Vector2Int filamentPosition)
        {
            Universe.GenerationSettings universeGenerationSettings = Universe.Instance.UniverseGenerationSettings;
            Filament.GenerationSettings filamentGenerationSettings = Universe.Instance.FilamentGenerationSettings;
            this.filamentPosition = filamentPosition;
            worldPosition = filamentPosition * filamentGenerationSettings.size;

            #region Main Generation
            FastNoiseLite fastNoiseLite = new FastNoiseLite();

            //General
            fastNoiseLite.SetNoiseType(FastNoiseLite.NoiseType.Cellular);
            fastNoiseLite.SetSeed(universeGenerationSettings.seed);
            fastNoiseLite.SetFrequency(0.04f);

            //Fractal
            fastNoiseLite.SetFractalType(FastNoiseLite.FractalType.FBm);
            fastNoiseLite.SetFractalOctaves(5);
            fastNoiseLite.SetFractalLacunarity(2.0f);
            fastNoiseLite.SetFractalGain(0.5f);
            fastNoiseLite.SetFractalWeightedStrength(0.3f);

            //Cellular
            fastNoiseLite.SetCellularDistanceFunction(FastNoiseLite.CellularDistanceFunction.EuclideanSq);
            fastNoiseLite.SetCellularReturnType(FastNoiseLite.CellularReturnType.Distance);
            fastNoiseLite.SetCellularJitter(1.0f);

            Color[] noiseColorMap = new Color[filamentGenerationSettings.size * filamentGenerationSettings.size];
            for (int y = 0; y < filamentGenerationSettings.size; y++)
            {
                for (int x = 0; x < filamentGenerationSettings.size; x++)
                {
                    float offsetX = - worldPosition.x;
                    float offsetY = - worldPosition.y;

                    float sampleX = x + offsetX;
                    float sampleY = y + offsetY;

                    float noiseValue = (fastNoiseLite.GetNoise(sampleX, sampleY) + 1) / 2;
                    noiseColorMap[y * filamentGenerationSettings.size + x] = new Color(noiseValue, noiseValue, noiseValue, 1.0f);
                }
            }

            map = TextureUtil.TextureFromColorMap(noiseColorMap, filamentGenerationSettings.size, filamentGenerationSettings.size);

            #endregion
        }

        public void Spawn()
        {
            filamentObject = GameObject.Instantiate(Universe.Instance.FilamentGenerationSettings.prefab);
            filamentObject.name = $"Filament ({filamentPosition.x}, {filamentPosition.y})";
            filamentObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            MapDisplay mapDisplay = filamentObject.GetComponentInChildren<MapDisplay>();
            mapDisplay.DrawTexture(map);
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(filamentObject);
        }
    }
}