namespace LooCast.System.Identification
{
    public interface IRegistryIdentifier : IGenericIdentifier<Registry<IIdentifier, IIdentifiable>>
    {
        #region Properties
        string KeyTypeID { get; }
        string ValueTypeID { get; }
        string RegistryID { get; }
        #endregion
    }
}