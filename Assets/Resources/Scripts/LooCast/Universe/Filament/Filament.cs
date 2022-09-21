using UnityEngine;
using System;

namespace LooCast.Filament
{
    using LooCast.Test;
    using LooCast.Util;

    public class Filament
    {
        public Vector2 WorldPosition => worldPosition;
        public Vector2Int FilamentPosition => filamentPosition;

        [SerializeField] private int size;
        [SerializeField] private Vector2Int filamentPosition;
        [SerializeField] private Vector2 worldPosition;

        private GameObject filamentObject;

        public Filament(Vector2Int filamentPosition, int size)
        {
            this.size = size;
            this.filamentPosition = filamentPosition;
            worldPosition = filamentPosition * size;
        }

        public void Spawn(GameObject prefab)
        {
            filamentObject = GameObject.Instantiate(prefab);
            filamentObject.name = $"Filament ({filamentPosition.x}, {filamentPosition.y})";
            filamentObject.transform.position = worldPosition * 10.0f;

            MapDisplay mapDisplay = filamentObject.GetComponentInChildren<MapDisplay>();
            //mapDisplay.DrawTexture(TextureUtil.TextureFromHeightMap(noiseMap.DataPointArray2D));
        }

        public void Despawn()
        {
            GameObject.DestroyImmediate(filamentObject);
        }
    }
}