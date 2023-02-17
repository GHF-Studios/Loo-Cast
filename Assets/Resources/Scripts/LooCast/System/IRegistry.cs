namespace LooCast.System
{
    using System.Identification;
    
    public interface IRegistry : IIdentifiable
    {
        #region Properties
        IRegistryIdentifier RegistryIdentifier { get; }
        #endregion
    }
}