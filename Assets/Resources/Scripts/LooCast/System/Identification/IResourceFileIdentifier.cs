namespace LooCast.System.Identification
{
    public interface IResourceFileIdentifier : IResourceObjectIdentifier
    {
        #region Properties
        string ResourceFileTypeID { get; }
        string ResourceFileGUID { get; }
        string ResourceFileID { get; }
        #endregion
    }
}