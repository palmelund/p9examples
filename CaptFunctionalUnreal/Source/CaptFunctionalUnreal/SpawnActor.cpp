// Fill out your copyright notice in the Description page of Project Settings.

#include "SpawnActor.h"


// Sets default values
ASpawnActor::ASpawnActor()
{
 	// Set this actor to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;

}

// Called when the game starts or when spawned
void ASpawnActor::BeginPlay()
{
	Super::BeginPlay();
	
}

// Called every frame
void ASpawnActor::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	auto realtime_seconds = GetWorld()->GetRealTimeSeconds();

	if (realtime_seconds >= last_spawn + spawnrate) {
		auto spawn = GetWorld()->SpawnActor<AActor>(enemy_blueprint);
		spawn->SetActorLocation(FVector(0, 0, 0));
		spawn->SetActorRotation(FRotator(0,90,0));
		last_spawn = realtime_seconds;
	}
}

