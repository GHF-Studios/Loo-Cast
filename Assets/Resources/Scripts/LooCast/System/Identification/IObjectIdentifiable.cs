namespace LooCast.System.Identification
{
    public interface IObjectIdentifiable : IInstanceIdentifiable
    {
        #region Properties
        IObjectIdentifier ObjectIdentifier { get; }
        #endregion
    }
}
