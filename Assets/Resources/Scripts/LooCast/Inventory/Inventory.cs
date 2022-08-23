using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Inventory
{
    using LooCast.Item;

    public sealed class Inventory : ScriptableObject
    {
        [SerializeField] private Item[] items;
        public ItemContainer itemContainer { get; private set; }

        private void OnValidate()
        {
            itemContainer = new ItemContainer(items);
            //Save(true);
        }

        private void OnEnable()
        {
            //Load();
        }

        private void OnDisable()
        {
            //Save();
        }

        public void Save(bool saveDefault = false)
        {
            throw new NotImplementedException();
        }

        public void Load()
        {
            throw new NotImplementedException();
        }
    }
}