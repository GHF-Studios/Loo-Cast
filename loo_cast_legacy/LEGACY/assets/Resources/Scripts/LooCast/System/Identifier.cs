using System;
using UnityEngine;

namespace LooCast.System
{
    [Serializable]
    public abstract class Identifier
    {
        #region Properties
        public string GUSID => gusid;
        #endregion
        
        #region Fields
        [SerializeField] private string gusid;
        #endregion

        #region Constructors
        public Identifier(string gusid)
        {
            this.gusid = gusid;
        }
        #endregion
    }
}
