using System;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public abstract class HierarchicalObjectPath
    {
        #region Properties
        public string GUSP => gusp;
        #endregion

        #region Fields
        [SerializeField] private string gusp;
        #endregion

        #region Constructors
        protected HierarchicalObjectPath(string gusp)
        {
            this.gusp = gusp;
        }
        #endregion
    }
}
