namespace LooCast.System.Identification
{
    public interface IObjectIdentifier : ICSharpInstanceIdentifier
    {
        #region Properties
        string ObjectTypeID { get; }
        string ObjectGUID { get; }
        string ObjectID { get; }
        #endregion
    }
}