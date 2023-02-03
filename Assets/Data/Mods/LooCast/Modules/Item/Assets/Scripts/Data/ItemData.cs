using UnityEngine;

namespace LooCast.Item.Data
{
    using LooCast.Data;

    public abstract class ItemData : ScriptableObject
    {
        public StringDataReference ItemName;
        public StringDataReference ItemDataName;
        public Sprite Sprite;
        public GameObject ItemObjectPrefab;

        public abstract Item CreateItem();
    }
}