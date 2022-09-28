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
                map = value;
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