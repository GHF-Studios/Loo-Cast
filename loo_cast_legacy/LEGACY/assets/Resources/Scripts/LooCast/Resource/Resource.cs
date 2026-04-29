using System;
using UnityEngine;

namespace LooCast.Resource
{
    [CreateAssetMenu(fileName = "Resource", menuName = "Data/Resource/Resource", order = 0)]
    public class Resource : ScriptableObject
    {
        #region Classes
        public enum Rarity
        {
            Common,
            Uncommon,
            Rare,
            Epic,
            Legendary
        }
        #endregion

        #region Properties
        public string ResourceName { get { return resoureceName; } }
        public Rarity ResourceRarity { get { return resourceRarity; } }
        #endregion

        #region Fields
        [SerializeField] private string resoureceName;
        [SerializeField] private Rarity resourceRarity;
        #endregion
    }
}
