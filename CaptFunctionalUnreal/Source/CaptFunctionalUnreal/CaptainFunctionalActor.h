// Fill out your copyright notice in the Description page of Project Settings.

#pragma once

#include "CoreMinimal.h"
#include "Engine.h"
#include "GameFramework/Character.h"
#include "Components/InputComponent.h"
#include "Components/BoxComponent.h"
#include "CaptainFunctionalActor.generated.h"

UCLASS()
class CAPTFUNCTIONALUNREAL_API ACaptainFunctionalActor : public ACharacter
{
	GENERATED_BODY()

public:
	// Sets default values for this character's properties
	ACaptainFunctionalActor();

	UPROPERTY(EDITANYWHERE) float speed;
	UPROPERTY(EDITANYWHERE) int health;

	UPROPERTY(EDITANYWHERE) float min_x_buffer;
	UPROPERTY(EDITANYWHERE) float min_y_buffer;
	UPROPERTY(EDITANYWHERE) float max_x_buffer;
	UPROPERTY(EDITANYWHERE) float max_y_buffer;

	UBoxComponent* Collider;

	// Called every frame
	virtual void Tick(float DeltaTime) override;

	// Called to bind functionality to input
	virtual void SetupPlayerInputComponent(class UInputComponent* PlayerInputComponent) override;

	virtual void NotifyActorBeginOverlap(AActor* Other) override;

	int GetHealth();
	void SetHealth(int hp);

protected:
	// Called when the game starts or when spawned
	virtual void BeginPlay() override;

	bool move_up_pressed;
	bool move_down_pressed;
	bool move_right_pressed;
	bool move_left_pressed;
	bool attack_pressed;

	FVector vector_min;
	FVector vector_max;
};
