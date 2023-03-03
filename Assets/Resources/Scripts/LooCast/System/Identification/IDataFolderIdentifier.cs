namespace LooCast.System.Identification
{
    public interface IDataFolderIdentifier : IDataObjectIdentifier
    {
        #region Properties
        string DataFolderTypeID { get; }
        string DataFolderGUID { get; }
        string DataFolderID { get; }
        #endregion
    }
}