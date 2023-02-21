namespace LooCast.System.Identification
{
    public interface IRegistryIdentifier : IIdentifier
    {
        #region Properties
        string KeyTypeID { get; }
        string ValueTypeID { get; }
        string RegistryID { get; }
        #endregion
    }
}