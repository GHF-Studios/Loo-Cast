using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Data
{
    public interface IDataFolder : IData
    {
        #region Properties
        Dictionary<string, IData> ContainedData { get; }
        List<DataFolder> ChildFolders { get; }
        #endregion
    }
}
