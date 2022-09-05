using System;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.Events;

namespace LooCast.Item
{
    using Data;
    using LooCast.Util;

    public class ItemContainer
    {
        protected Dictionary<int, ItemContainerSlot> itemSlots;
        public UnityEvent OnChange
        {
            get
            {
                return onChange;
            }
        }
        public GameObject OriginObject
        {
            get
            {
                return originObject;
            }
        }
        private UnityEvent onChange;
        private GameObject originObject;

        public ItemContainer(int slotCount, GameObject originObject = null)
        {
            if (slotCount <= 0)
            {
                throw new ArgumentOutOfRangeException("Slot Count must be greater than 0!");
            }

            onChange = new UnityEvent();

            Clear(slotCount);

            if (originObject != null)
            {
                this.originObject = originObject;
            }
        }

        public ItemContainer(Item[] items, GameObject originObject = null)
        {
            if (items == null)
            {
                throw new ArgumentNullException("Items cannot be null!");
            }
            if (items.Length == 0)
            {
                throw new ArgumentOutOfRangeException("Items must have atleast one entry!");
            }
             
            onChange = new UnityEvent();
            
            Clear(itemSlots.Count);
            foreach (Item item in items)
            {
                AddItem(item, out Item remainingItem);
            }

            if (originObject != null)
            {
                this.originObject = originObject;
            }
        }

        public virtual void AddItem(Item item, out Item remainingItem)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }

            if (item is CountableItem)
            {
                AddItem((CountableItem)item, out CountableItem remainingCountableItem);
                remainingItem = remainingCountableItem;
                OnChange.Invoke();
                return;
            }
            else if (item is AmountableItem)
            {
                AddItem((AmountableItem)item, out AmountableItem remainingAmountableItem);
                remainingItem = remainingAmountableItem;
                OnChange.Invoke();
                return;
            }
            else if (item is UniqueItem)
            {
                AddItem((UniqueItem)item, out UniqueItem remainingUniqueItem);
                remainingItem = remainingUniqueItem;
                OnChange.Invoke();
                return;
            }
            else
            {
                throw new NotSupportedException("Unsupported Item Type!");
            }
        }

        protected void AddItem(CountableItem countableItem, out CountableItem remainingCountableItem)
        {
            remainingCountableItem = countableItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingCountableItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingCountableItem.UndropItem();
                    }
                    remainingCountableItem.ContainItem(this);
                    itemSlots[i].ItemContent = remainingCountableItem;
                    remainingCountableItem = null;
                    break;
                }
                else if (itemSlots[i].Equals(remainingCountableItem))
                {
                    CountableItem countableItemSlot = (CountableItem)itemSlots[i].ItemContent;
                    if (!countableItemSlot.IsFull())
                    {
                        int freeCount = countableItemSlot.GetFreeCount();
                        if (freeCount >= remainingCountableItem.Count)
                        {
                            countableItemSlot.Count += remainingCountableItem.Count;
                            remainingCountableItem = null;
                            break;
                        }
                        else
                        {
                            countableItemSlot.Count = countableItemSlot.MaxCount;
                            remainingCountableItem.Count -= freeCount;
                        }
                    }
                }
            }
            OnChange.Invoke();
        }

        protected void AddItem(AmountableItem amountableItem, out AmountableItem remainingAmountableItem)
        {
            remainingAmountableItem = amountableItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingAmountableItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingAmountableItem.UndropItem();
                    }
                    remainingAmountableItem.ContainItem(this);
                    itemSlots[i].ItemContent = remainingAmountableItem;
                    remainingAmountableItem = null;
                    break;
                }
                else if (itemSlots[i].Equals(remainingAmountableItem))
                {
                    AmountableItem amountableItemSlot = (AmountableItem)itemSlots[i].ItemContent;
                    if (!amountableItemSlot.IsFull())
                    {
                        float freeAmount = amountableItemSlot.GetFreeAmount();
                        if (freeAmount >= remainingAmountableItem.Amount)
                        {
                            amountableItemSlot.Amount += remainingAmountableItem.Amount;
                            remainingAmountableItem = null;
                            break;
                        }
                        else
                        {
                            amountableItemSlot.Amount = amountableItemSlot.MaxAmount;
                            remainingAmountableItem.Amount -= freeAmount;
                        }
                    }
                }
            }
            OnChange.Invoke();
        }

        protected void AddItem(UniqueItem uniqueItem, out UniqueItem remainingUniqueItem)
        {
            remainingUniqueItem = uniqueItem;
            for (int i = 0; i < itemSlots.Count; i++)
            {
                if (itemSlots[i].ItemContent == null)
                {
                    if (remainingUniqueItem.ItemContainmentState == Item.ContainmentState.Dropped)
                    {
                        remainingUniqueItem.UndropItem();
                    }
                    remainingUniqueItem.ContainItem(this);
                    itemSlots[i].ItemContent = remainingUniqueItem;
                    remainingUniqueItem = null;
                    OnChange.Invoke();
                    return;
                }
            }
        }

        public virtual void SetItem(int slotID, Item item)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            itemSlots[slotID].ItemContent = item;
            OnChange.Invoke();
        }

        public Item GetItem(int slotID)
        {
            if (!IsValidSlot(slotID))
            {
                throw new ArgumentOutOfRangeException($"Invalid slot! Slot must be between 0 {itemSlots.Count - 1}!");
            }
            bool success = itemSlots.TryGetValue(slotID, out ItemContainerSlot slot);
            if (!success)
            {
                throw new Exception($"Unable to get Item at SlotID: {slotID}");
            }
            return slot.ItemContent;
        }

        public Item[] GetItems()
        {
            return itemSlots.GetItems();
        }

        public virtual bool Contains(Item item)
        {
            if (item == null)
            {
                throw new ArgumentNullException("Item cannot be null!");
            }
            foreach (KeyValuePair<int, ItemContainerSlot> slot in itemSlots)
            {
                if (slot.Value.ItemContent.Equals(item))
                {
                    return true;
                }
            }
            return false;
        }

        public bool Contains(int slotID)
        {
            return itemSlots.ContainsKey(slotID);
        }

        public void Clear(int slotCount)
        {
            itemSlots = new Dictionary<int, ItemContainerSlot>();

            for (int i = 0; i < slotCount; i++)
            {
                RemoveSlot(i);
                AddSlot(i);
            }

            OnChange.Invoke();
        }

        public bool IsValidSlot(int slot)
        {
            return slot < itemSlots.Count && slot >= 0;
        }

        public void AddSlot(int slotID)
        {
            if (itemSlots.ContainsKey(slotID))
            {
                throw new ArgumentException("SlotID is already occupied!");
            }
            itemSlots.Add(slotID, new ItemContainerSlot());
        }

        public void RemoveSlot(int slotID)
        {
            itemSlots.Remove(slotID);
        }

        public bool IsBoundToObject()
        {
            return originObject != null;
        }

        public override string ToString()
        {
            string message = "";
            for (int i = 0; i < itemSlots.Count; i++)
            {
                message += $"Slot {i}:\t";
                if (itemSlots[i] != null)
                {
                    message += $"{itemSlots[i].ItemContent}\n";
                }
                else
                {
                    message += "null\n";
                }
            }
            return message;
        }
    }
}