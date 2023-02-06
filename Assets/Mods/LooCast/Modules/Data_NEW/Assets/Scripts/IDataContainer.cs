using System.Collections.Generic;

namespace LooCast.Data_NEW
{
    public interface IDataContainer : IData
    {
        #region Properties
        Dictionary<string, IData> ContainedData { get; }
        Dictionary<string, IDataContainer> ContainedFolders { get; }
        #endregion
    }
}
