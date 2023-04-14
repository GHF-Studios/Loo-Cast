using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.MetaData
{
    [Serializable]
    public abstract class RegistryMetaData : SystemObjectMetaData
    {
        #region Properties
#nullable enable
        public IRegistry? BaseRegistry { get; set; }
#nullable disable
        #endregion
    }
}
