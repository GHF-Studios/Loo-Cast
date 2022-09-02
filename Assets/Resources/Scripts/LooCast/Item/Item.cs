using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        private static Dictionary<int, Item> itemDictionary = new Dictionary<int, Item>();
        private static int IDCounter = 0;
        public int ID { get; private set; }
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }
        public GameObject ItemObjectPrefab { get; protected set; }
        public ItemObject ItemObject { get; private set; }
        public ItemContainer<Item> ItemContainer { get; private set; }
        public UnityEvent OnFinalize { get; private set; }
        public bool IsDropped
        {
            get
            {
                return ItemObject != null;
            }
        }
        public UnityEvent OnSpawn { get; private set; }
        public UnityEvent<GameObject> OnPickup { get; private set; }
        public ItemContainmentState ItemContainmentState { get; private set; }

        public Item(ItemData data)
        {
            OnFinalize = new UnityEvent();
            OnSpawn = new UnityEvent();
            OnPickup = new UnityEvent<GameObject>();

            ItemContainmentState = ItemContainmentState.Standalone;

            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName.Value;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            ItemObject = null;
            ItemContainer = null;

            itemDictionary.Add(ID, this);
        }

        public Item(ItemData data, ItemObject itemObject)
        {
            OnFinalize = new UnityEvent();
            OnSpawn = new UnityEvent();
            OnPickup = new UnityEvent<GameObject>();

            ItemContainmentState = ItemContainmentState.Dropped;

            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName.Value;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            ItemObject = itemObject;
            ItemContainer = null;

            itemDictionary.Add(ID, this);

            OnSpawn.Invoke();
        }

        public Item(ItemData data, ItemContainer<Item> itemContainer, GameObject itemContainerOrigin)
        {
            OnFinalize = new UnityEvent();
            OnSpawn = new UnityEvent();
            OnPickup = new UnityEvent<GameObject>();

            ItemContainmentState = ItemContainmentState.Contained;

            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName.Value;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            ItemObject = null;
            ItemContainer = itemContainer;

            itemDictionary.Add(ID, this);

            OnPickup.Invoke(itemContainerOrigin);
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

        public void DropItem(Vector3 spawnPosition)
        {
            if (ItemContainmentState == ItemContainmentState.Dropped)
            {
                throw new Exception("Can not spawn Item: Item has already dropped!");
            }
            if (ItemContainmentState == ItemContainmentState.Contained)
            {
                UncontainItem();
            }
            ItemObject = GameObject.Instantiate(ItemObjectPrefab, spawnPosition, Quaternion.identity).GetComponent<ItemObject>();
            ItemObject.Item = this;
            ItemContainer = null;
            ItemContainmentState = ItemContainmentState.Dropped;
            OnSpawn.Invoke();
        }

        public void UndropItem()
        {
            GameObject.Destroy(ItemObject.gameObject);
            ItemObject = null;
            ItemContainmentState = ItemContainmentState.Standalone;
        }

        public void ContainItem(ItemContainer<Item> itemContainer, GameObject itemContainerOrigin)
        {
            if (ItemContainmentState == ItemContainmentState.Contained)
            {
                throw new Exception("Can not contain Item: Item is already contained!");
            }
            if (ItemContainmentState == ItemContainmentState.Dropped)
            {
                UndropItem();
            }
            ItemContainer = itemContainer;
            ItemContainmentState = ItemContainmentState.Contained;
            OnPickup.Invoke(itemContainerOrigin);
        }

        public void UncontainItem()
        {
            ItemContainer = null;
            ItemContainmentState = ItemContainmentState.Standalone;
        }

        public virtual void Use()
        {

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