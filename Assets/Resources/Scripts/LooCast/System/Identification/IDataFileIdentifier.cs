namespace LooCast.System.Identification
{
    public interface IDataFileIdentifier : IDataObjectIdentifier
    {
        #region Properties
        string DataFileTypeID { get; }
        string DataFileGUID { get; }
        string DataFileID { get; }
        #endregion
    }
}