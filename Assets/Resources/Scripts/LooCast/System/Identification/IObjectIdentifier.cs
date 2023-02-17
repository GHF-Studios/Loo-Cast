namespace LooCast.System.Identification
{
    public interface IObjectIdentifier : IInstanceIdentifier
    {
        #region Properties
        string ObjectTypeID { get; }
        string ObjectGUID { get; }
        string ObjectID { get; }
        #endregion
    }
}
