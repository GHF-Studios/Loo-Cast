using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item.Data
{
    [CreateAssetMenu(fileName = "ItemDatas", menuName = "Data/Item/ItemDatas", order = 0)]
    public sealed class ItemDatas : ScriptableObject
    {
        [SerializeField] private List<ItemData> itemDataList;
        private Dictionary<string, ItemData> itemDataDictionary;

        private void OnValidate()
        {
            itemDataDictionary = new Dictionary<string, ItemData>();
            foreach (ItemData itemData in itemDataList)
            {
                if (itemDataDictionary.ContainsKey(itemData.ItemName.Value))
                {
                    throw new Exception($"Already contains item {itemData.ItemName.Value}!");
                }
                itemDataDictionary.Add(itemData.ItemName.Value, itemData);
            }
        }

        public void AddItemData(ItemData itemData)
        {
            if (itemDataDictionary.ContainsKey(itemData.ItemName.Value))
            {
                throw new Exception($"Already contains item {itemData.ItemName.Value}!");
            }
            itemDataList.Add(itemData);
            itemDataDictionary.Add(itemData.ItemName.Value, itemData);
        }

        public ItemData GetItemData(string name)
        {
            if (itemDataDictionary.ContainsKey(name))
            {
                itemDataDictionary.TryGetValue(name, out ItemData itemData);
                return itemData;
            }
            throw new ArgumentException($"No ItemData named '{name}' could be found!");
        }
    }
}