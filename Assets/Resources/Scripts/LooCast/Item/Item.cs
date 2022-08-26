using UnityEngine;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        private static int IDCounter = 0;
        public int ID { get; private set; }
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }
        public GameObject ItemObjectPrefab { get; protected set; }
        public float MinObjectScale { get; protected set; }
        public float MaxObjectScale { get; protected set; }

        public Item(ItemData data)
        {
            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            MinObjectScale = data.MinObjectScale;
            MaxObjectScale = data.MaxObjectScale;
        }

        public ItemObject SpawnItem(Vector3 spawnPosition)
        {
            ItemObject itemObject = GameObject.Instantiate(ItemObjectPrefab, spawnPosition, Quaternion.identity).GetComponent<ItemObject>();
            itemObject.Item = this;
            return itemObject;
        }

        public override string ToString()
        {
            return Name;
        }

        public override bool Equals(object obj)
        {
            Item item = (Item)obj;
            if (item != null && item.Name == Name)
            {
                return true;
            }
            return false;
        }

        public override int GetHashCode()
        {
            return ID;
        }
    }
}