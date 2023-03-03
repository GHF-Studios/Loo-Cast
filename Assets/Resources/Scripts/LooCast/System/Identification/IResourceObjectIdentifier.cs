namespace LooCast.System.Identification
{
    public interface IResourceObjectIdentifier : IResourceIdentifier
    {
        #region Properties
        string ResourceObjectTypeID { get; }
        string ResourceObjectGUID { get; }
        string ResourceObjectID { get; }
        #endregion
    }
}