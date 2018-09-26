// Fill out your copyright notice in the Description page of Project Settings.

#include "EnemyAttackComponent.h"


// Sets default values for this component's properties
UEnemyAttackComponent::UEnemyAttackComponent()
{
	// Set this component to be initialized when the game starts, and to be ticked every frame.  You can turn these features
	// off to improve performance if you don't need them.
	PrimaryComponentTick.bCanEverTick = true;

	// ...
}


// Called when the game starts
void UEnemyAttackComponent::BeginPlay()
{
	Super::BeginPlay();

	// ...
	
}


// Called every frame
void UEnemyAttackComponent::TickComponent(float DeltaTime, ELevelTick TickType, FActorComponentTickFunction* ThisTickFunction)
{
	Super::TickComponent(DeltaTime, TickType, ThisTickFunction);

	auto loc = GetOwner()->GetActorLocation();
	loc.Y -= speed * DeltaTime;
	GetOwner()->SetActorLocation(loc);
	// ...
}

