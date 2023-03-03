using System;

namespace LooCast.System.Identification
{
    public interface IDataFolderIdentifiable : IDataObjectIdentifiable
    {
        #region Properties
        IDataFolderIdentifier DataFolderIdentifier { get; }
        #endregion
    }
}
