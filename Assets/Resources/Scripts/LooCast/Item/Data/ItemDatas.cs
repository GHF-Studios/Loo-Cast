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
                itemDataDictionary.Add(itemData.name, itemData);
            }
        }

        public void AddItemData(ItemData itemData)
        {
            if (itemDataDictionary.ContainsKey(itemData.name))
            {
                throw new Exception($"Already contains item {itemData.name}!");
            }
            itemDataList.Add(itemData);
            itemDataDictionary.Add(itemData.name, itemData);
        }
    }
}