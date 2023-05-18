using System;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public abstract class Identifier : IObjectIdentifier
    {
        #region Properties
        public string GUSID => gusid;
        #endregion
        
        #region Fields
        [SerializeField] private string gusid;
        #endregion

        #region Constructors
        protected Identifier(string gusid)
        {
            this.gusid = gusid;
        }
        #endregion
    }
}
