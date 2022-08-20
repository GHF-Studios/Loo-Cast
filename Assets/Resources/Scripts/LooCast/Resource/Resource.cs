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

        #region Fields
        public Rarity ResourceRarity;
        #endregion
    }
}
