using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;

    public abstract class Item
    {
        #region Data
        public ItemData ItemData { get; private set; }
        #endregion

        public enum ContainmentState
        {
            Contained,
            Dropped,
            Standalone
        }

        private static Dictionary<int, Item> itemDictionary = new Dictionary<int, Item>();
        private static int IDCounter = 0;
        public int ID { get; private set; }
        public string Name { get; protected set; }
        public Sprite Sprite { get; protected set; }
        public GameObject ItemObjectPrefab { get; protected set; }
        public ItemObject ItemObject { get; private set; }
        public ItemContainer ItemContainer { get; private set; }
        public UnityEvent OnFinalize { get; private set; }
        public UnityEvent OnContainmentStateChange { get; private set; }
        public ContainmentState ItemContainmentState
        {
            get
            {
                return itemContainmentState;
            }

            protected set
            {
                itemContainmentState = value;
                OnContainmentStateChange.Invoke();
            }
        }
        private ContainmentState itemContainmentState;

        public Item(ItemData data)
        {
            ItemData = data;

            OnFinalize = new UnityEvent();
            OnContainmentStateChange = new UnityEvent();

            ItemContainmentState = ContainmentState.Standalone;

            ID = IDCounter;
            IDCounter++;
            Name = data.ItemName.Value;
            Sprite = data.Sprite;
            ItemObjectPrefab = data.ItemObjectPrefab;
            ItemObject = null;
            ItemContainer = null;

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

        public void DropItem(Vector3 spawnPosition)
        {
            if (ItemContainmentState == ContainmentState.Dropped)
            {
                throw new Exception("Can not drop Item: Item is already dropped!");
            }
            if (ItemContainmentState == ContainmentState.Contained)
            {
                throw new Exception("Can not drop Item: Item is contained!");
            }
            ItemObject = GameObject.Instantiate(ItemObjectPrefab, spawnPosition, Quaternion.identity).GetComponent<ItemObject>();
            ItemObject.Item = this;
            ItemContainer = null;
            ItemContainmentState = ContainmentState.Dropped;
        }

        public void UndropItem()
        {
            if (ItemContainmentState == ContainmentState.Standalone)
            {
                throw new Exception("Can not undrop Item: Item is alreay standalone!");
            }
            if (ItemContainmentState == ContainmentState.Contained)
            {
                throw new Exception("Can not undrop Item: Item is contained!");
            }
            GameObject.Destroy(ItemObject.gameObject);
            ItemObject = null;
            ItemContainmentState = ContainmentState.Standalone;
        }

        public void ContainItem(ItemContainer itemContainer)
        {
            if (ItemContainmentState == ContainmentState.Contained)
            {
                throw new Exception("Can not contain Item: Item is already contained!");
            }
            if (ItemContainmentState == ContainmentState.Dropped)
            {
                throw new Exception("Can not contain Item: Item is dropped!");
            }
            ItemContainer = itemContainer;
            ItemContainmentState = ContainmentState.Contained;
        }

        public void UncontainItem()
        {
            if (ItemContainmentState == ContainmentState.Standalone)
            {
                throw new Exception("Can not uncontain Item: Item is alreay standalone!");
            }
            if (ItemContainmentState == ContainmentState.Dropped)
            {
                throw new Exception("Can not uncontain Item: Item is dropped!");
            }
            ItemContainer = null;
            ItemContainmentState = ContainmentState.Standalone;
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