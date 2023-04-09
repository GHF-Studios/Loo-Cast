using System;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public abstract class HierarchyElementPath : Identifier
    {
        #region Properties
        public string GUSP => gusp;
        #endregion

        #region Fields
        [SerializeField] private string gusp;
        #endregion

        #region Constructors
        protected HierarchyElementPath(string gusp) : base(gusp)
        {
            this.gusp = gusp;
        }
        #endregion
    }
}
