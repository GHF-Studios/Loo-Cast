namespace LooCast.System
{
    using System.Identification;
    
    public interface IRegistry
    {
        #region Properties
        IRegistryIdentifier RegistryIdentifier { get; }
        #endregion
    }
}