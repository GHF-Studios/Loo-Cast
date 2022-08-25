using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Item.Data
{
    [CreateAssetMenu(fileName = "ItemObjectPrefabs", menuName = "Data/Item/ItemObjectPrefabs", order = 0)]
    public sealed class ItemObjectPrefabs : ScriptableObject
    {
        [SerializeField] private List<GameObject> itemObjectPrefabList;
        private Dictionary<string, GameObject> itemObjectPrefabDictionary;

        private void OnValidate()
        {
            itemObjectPrefabDictionary = new Dictionary<string, GameObject>();
            foreach (GameObject itemObjectPrefab in itemObjectPrefabList)
            {
                itemObjectPrefabDictionary.Add(itemObjectPrefab.name, itemObjectPrefab);
            }
        }

        public void AddItemObjectPrefab(GameObject itemObjectPrefab)
        {
            if (itemObjectPrefabDictionary.ContainsKey(itemObjectPrefab.name))
            {
                throw new Exception($"Already contains item {itemObjectPrefab.name}!");
            }
            itemObjectPrefabList.Add(itemObjectPrefab);
            itemObjectPrefabDictionary.Add(itemObjectPrefab.name, itemObjectPrefab);
        }

        public GameObject GetItemObjectPrefab(string name)
        {
            if (itemObjectPrefabDictionary.ContainsKey(name))
            {
                itemObjectPrefabDictionary.TryGetValue(name, out GameObject itemObjectPrefab);
                return itemObjectPrefab;
            }
            throw new ArgumentException($"No ItemData named '{name}' could be found!");
        }
    } 
}
