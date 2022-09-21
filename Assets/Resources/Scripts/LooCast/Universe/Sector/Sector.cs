using UnityEngine;
using System;

namespace LooCast.Sector
{
    using LooCast.Test;
    using LooCast.Util;

    public class Sector
    {
        public Vector2 WorldPosition => worldPosition;
        public Vector2Int SectorPosition => sectorPosition;

        [SerializeField] private int size;
        [SerializeField] private Vector2Int sectorPosition;
        [SerializeField] private Vector2 worldPosition;

        private GameObject sectorObject;

        public Sector(Vector2Int sectorPosition, int size)
        {
            this.size = size;
            this.sectorPosition = sectorPosition;
            worldPosition = sectorPosition * size;
        }

        public void Spawn(GameObject prefab)
        {
            sectorObject = GameObject.Instantiate(prefab);
            sectorObject.name = $"Sector ({sectorPosition.x}, {sectorPosition.y})";
            sectorObject.transform.position = worldPosition * 10.0f;

            MapDisplay mapDisplay = sectorObject.GetComponentInChildren<MapDisplay>();
            //mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(sectorObject);
        }
    }
}