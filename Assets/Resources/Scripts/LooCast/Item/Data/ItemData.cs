using UnityEngine;

namespace LooCast.Item.Data
{
    public abstract class ItemData : ScriptableObject
    {
        public string Name;
        public Sprite Sprite;
        public GameObject ItemObjectPrefab;
    }
}