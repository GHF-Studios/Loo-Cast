namespace LooCast.System
{
    using System.Identification;
    
    public interface IGameObject : IInstance
    {
        #region Properties
        IGameObjectIdentifier GameObjectIdentifier { get; }
        #endregion
    }
}