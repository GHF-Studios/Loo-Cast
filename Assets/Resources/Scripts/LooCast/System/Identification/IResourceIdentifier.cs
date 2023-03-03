namespace LooCast.System.Identification
{
    public interface IResourceIdentifier : IObjectIdentifier
    {
        #region Properties
        string ResourceTypeID { get; }
        string ResourceGUID { get; }
        string ResourceID { get; }
        #endregion
    }
}