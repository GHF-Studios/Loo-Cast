using System;
using UnityEngine;
using LooCast;

namespace LooCast.Core.Manager
{
    public abstract class CoreModuleManager : ModuleManager
    {
        #region Properties
        public abstract ModuleManager[] ModuleManagers { get; }
        #endregion
    }
}