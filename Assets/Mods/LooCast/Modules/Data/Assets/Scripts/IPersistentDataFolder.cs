using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Data
{
    public interface IPersistentDataFolder : IPersistentData
    {
        #region Properties
        Dictionary<string, IPersistentData> ContainedData { get; }
        List<IPersistentDataFolder> ChildFolders { get; }
        #endregion
    }
}
