using System;
using UnityEngine;
using LooCast;

namespace LooCast.Core.Manager
{ 
    public abstract class ModuleManager : Manager
    {
        #region Properties
        public abstract SubModuleManager[] SubModuleManagers { get; }
        #endregion
    }
}