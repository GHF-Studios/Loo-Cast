using UnityEngine;

namespace LooCast.Item.Data
{
    public abstract class ItemData : ScriptableObject
    {
        public string ItemName;
        public Sprite Sprite;
        public GameObject ItemObjectPrefab;
        public float MinObjectScale;
        public float MaxObjectScale;
    }
}