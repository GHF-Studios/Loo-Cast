using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Player;

    public abstract class Item
    {
        private static Dictionary<int, Item> itemDictionary = new Dictionary<int, Item>();
        private static int IDCounter = 0;
        public int ID { get; private set; }
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }
        public GameObject ItemObjectPrefab { get; protected set; }
        public ItemObject ItemObject { get; private set; }
        public UnityEvent OnFinalize { get; private set; }
        public bool IsDropped
        {
            get
            {
                return ItemObject != null;
            }
        }
        public UnityEvent OnDrop { get; private set; }
        public UnityEvent<GameObject> OnPickup { get; private set; }

        public Item(ItemData data, ItemObject itemObject)
        {
            OnFinalize = new UnityEvent();
            OnDrop = new UnityEvent();
            OnPickup = new UnityEvent<GameObject>();

            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName.Value;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            ItemObject = itemObject;

            itemDictionary.Add(ID, this);
        }

        ~Item()
        {
            OnFinalize.Invoke();
            itemDictionary.Remove(ID);
        }

        public static Item GetItem(int ID)
        {
            if (!itemDictionary.ContainsKey(ID))
            {
                throw new ArgumentOutOfRangeException($"No item found with ID {ID}");
            }
            itemDictionary.TryGetValue(ID, out Item item);
            return item;
        }

        public void SpawnItem(Vector3 spawnPosition)
        {
            ItemObject = GameObject.Instantiate(ItemObjectPrefab, spawnPosition, Quaternion.identity).GetComponent<ItemObject>();
            ItemObject.Item = this;
            OnDrop.Invoke();
        }

        public void DespawnItem(GameObject origin)
        {
            GameObject.Destroy(ItemObject.gameObject);
            ItemObject = null;
            OnPickup.Invoke(origin);
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