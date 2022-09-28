using UnityEngine;
using System;

namespace LooCast.Universe.Region
{
    using Filament;
    using LooCast.Noise;
    using LooCast.Util;
    using LooCast.Test;
    using LooCast.Math.Map;

    [Serializable]
    public class Region
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
        public Vector2Int RegionPosition => regionPosition;

        [SerializeField] private Vector2Int sectorPosition;
        [SerializeField] private Vector2Int regionPosition;
        [SerializeField] private Vector2Int worldPosition;

        private GameObject regionObject;

        public Region(Vector2Int sectorPosition, Vector2Int regionPosition)
        {
            this.sectorPosition = sectorPosition;
            this.regionPosition = regionPosition;
            worldPosition = regionPosition * Universe.Instance.RegionGenerationSettings.size;
        }

        public void Spawn()
        {
            regionObject = GameObject.Instantiate(Universe.Instance.RegionGenerationSettings.prefab);
            regionObject.name = $"Region ({regionPosition.x}, {regionPosition.y})";
            regionObject.transform.position = new Vector3(worldPosition.x, worldPosition.y, 0.0f) * 10.0f;

            //MapDisplay mapDisplay = regionObject.GetComponentInChildren<MapDisplay>();
            //mapDisplay.DrawTexture();
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(regionObject);
        }
    }
}